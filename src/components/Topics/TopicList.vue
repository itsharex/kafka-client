<script setup lang="ts">
import { Fzf } from 'fzf';
import { computed, ref } from 'vue';
import { ArrowPathIcon, PlusIcon } from '@heroicons/vue/16/solid';
import {
	Dialog,
	DialogTrigger,
	DialogContent,
	DialogHeader,
	DialogFooter,
	DialogTitle,
	DialogDescription,
} from '@/components/ui/dialog';

import Input from '@/components/ui/input/Input.vue';
import Button from '@/components/ui/button/Button.vue';
import Label from '@/components/ui/label/Label.vue';

import { createTopic, deleteTopic as adminDeleteTopic, TopicInfo } from '@/lib/kafka';
import { Command, CommandEmpty, CommandInput, CommandItem, CommandList } from '@/components/ui/command';
import { useToast } from '@/components/ui/toast';
import { TrashIcon } from 'lucide-vue-next';
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from '@/components/ui/alert-dialog';

const props = defineProps<{ topics: TopicInfo[], error: string }>();
const {toast} = useToast();
const newTopicName = ref<string>();
const newTopicPartitions = ref(3);
const onEvent = defineEmits(['refresh']);
const selectedTopicModel = defineModel<TopicInfo | undefined>('selectedTopic')
const topicsIndex = computed(() => new Fzf(props.topics, { selector: (d) => d.name, sort: true }));
// const search = ref("");
const filterTopicsList = (search: string) =>
	topicsIndex.value
		.find(search)
		.sort((a, b) => b.score - a.score)
		.map((e) => e.item); 

const isNewTopicDialogOpen = ref(false);
const createNewTopic = async () => {
	if (!newTopicName.value) {
		return;
	}

	await createTopic({topic: newTopicName.value, partitions: newTopicPartitions.value, config: []})
	newTopicName.value = "";
	newTopicPartitions.value = 3;
	onEvent("refresh");
	isNewTopicDialogOpen.value = false;
}

const deleteTopic = (topic: string) => {
	adminDeleteTopic(topic)
		.then((deletedTopic) => {
			toast({title: "Success", description: `Topic '${deletedTopic}' deleted!`});
			onEvent("refresh");
		})
		.catch(err => toast({title: "Error", description: err, variant: "destructive"}));
}
</script>
<template>
	<div
		class="flex items-center justify-between leading-none py-1 px-2 border-t border-muted">
		<h4 class="uppercase text-xs tracking-wide font-bold">Topics</h4>
		<div class="space-x-2">
			<Dialog v-model:open="isNewTopicDialogOpen">
				<DialogTrigger as-child>
					<Button variant="outline" size="xs">
						<PlusIcon class="w-4 h-4" />
					</Button>
				</DialogTrigger>
				<DialogContent class="sm:max-w-[425px]">
					<DialogHeader>
						<DialogTitle>Create Topic</DialogTitle>
						<DialogDescription>
							Give topic name and partitions count
						</DialogDescription>
					</DialogHeader>
					<div class="grid gap-5 py-4">
						<div class="grid gap-2">
							<Label for="topic_name">
								Name
							</Label>
							<Input id="topic_name" v-model="newTopicName" />
						</div>
						<div class="grid gap-1">
							<Label for="partitions_count">
								Partition Count
							</Label>
							<Input id="partitions_count"  type="number" v-model="newTopicPartitions"  />
						</div>
					</div>
					<DialogFooter>
						<Button type="submit" @click="createNewTopic">
							Create
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>

			<Button variant="outline" size="xs" @click="onEvent('refresh')">
				<ArrowPathIcon class="block w-4 h-4" />
			</Button>
		</div>
	</div>
	<div class="px-2">
		<Command v-model="selectedTopicModel" :filter-function="(_, search) => filterTopicsList(search)">
			<CommandInput id="search-input" placeholder="Search Topics..." />
			<p v-if="props.error" class="-mx-2 px-2 py-1 leading-tight bg-error-100 text-error-800 my-2"
				v-text="props.error"></p>
			<CommandList>
				<CommandEmpty>
					<p v-if="topics.length > 0">
						No Topics Match the search keywords. Try clearing search.
					</p>
					<p v-else>
						No topics found.
						<Button @click="onEvent('refresh')" size="sm">
							Load Topics
						</Button>
					</p>
				</CommandEmpty>
				<CommandItem v-for="topic of topics" :key="topic.name"
					class="-mx-2 px-4 py-2 flex space-x-2 hover:bg-neutral-100 dark:hover:bg-neutral-800 ui-checked:bg-primary ui-checked:text-white cursor-pointer"
					:value="topic">
					<div class="flex-1">
						<code class="text-xs leading-none font-bold" v-text="topic.name"></code>
						<p class="text-opacity-25 font-medium text-xs uppercase">
							Partitions: {{ topic.partitions.length }}, Replicas:
							{{
								topic.partitions.map((p) => p.replicas.length).reduce((a, b) => a + b, 0) / topic.partitions.length
							}}
						</p>
					</div>
					<AlertDialog> 
						<AlertDialogTrigger as-child>
							<Button variant="outline" size="sm">
								<TrashIcon class="block w-4 h-4"></TrashIcon>
							</Button>
						</AlertDialogTrigger>
						<AlertDialogContent>
							<AlertDialogHeader>
								<AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
								<AlertDialogDescription>
									This action cannot be undone. This will permanently delete the topic and its data.
								</AlertDialogDescription>
							</AlertDialogHeader>
							<AlertDialogFooter>
								<AlertDialogCancel>No way, Take me back!</AlertDialogCancel>
								<AlertDialogAction @click="() => deleteTopic(topic.name)">Yes, Absolutely!</AlertDialogAction>
							</AlertDialogFooter>
						</AlertDialogContent>
					</AlertDialog>
				</CommandItem>
			</CommandList>
		</Command>
	</div>
</template>
