<script setup lang="ts">
import { Fzf } from 'fzf';
import { computed, ref } from 'vue';
import { RadioGroup, RadioGroupOption } from '@headlessui/vue';
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

import { createTopic, TopicInfo } from '@/lib/kafka';

const props = defineProps<{ topics: TopicInfo[], error: string }>();
const newTopicName = ref<string>();
const newTopicPartitions = ref(3);
const onEvent = defineEmits(['refresh']);
const selectedTopicModel = defineModel<TopicInfo | undefined>('selectedTopic')
const topicsIndex = computed(() => new Fzf(props.topics, { selector: (d) => d.name, sort: true }));
const search = ref("");
const filteredTopicsList = computed(() =>
	topicsIndex.value
		.find(search.value)
		.sort((a, b) => b.score - a.score)
		.map((e) => e.item)
); 

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
	<div class="px-2 py-2 border-t border-muted">
		<Input id="search-input" class="w-full form-input py-2 rounded" v-model="search"
			placeholder="Search Topics..." />
		<p v-if="props.error" class="-mx-2 px-2 py-1 leading-tight bg-error-100 text-error-800 my-2"
			v-text="props.error"></p>
	</div>
	<div class="px-2">
		<RadioGroup v-if="filteredTopicsList.length > 0" v-model="selectedTopicModel">
			<RadioGroupOption v-for="topic of filteredTopicsList" :key="topic.name"
				class="-mx-2 px-4 py-1 hover:bg-neutral-100 dark:hover:bg-neutral-800 ui-checked:bg-primary ui-checked:text-white cursor-pointer"
				:value="topic">
				<code class="text-xs leading-none font-bold" v-text="topic.name"></code>
				<p class="text-opacity-25 font-medium text-xs uppercase">
					Partitions: {{ topic.partitions.length }}, Replicas:
					{{
						topic.partitions.map((p) => p.replicas.length).reduce((a, b) => a + b, 0) / topic.partitions.length
					}}
				</p>
			</RadioGroupOption>
		</RadioGroup>

		<p v-else-if="topics.length > 0">
			No Topics Match the search keywords. <Button variant="link" class="p-0 h-auto" @click.prevent="search = ''">Clear</Button> search keyword.
		</p>
		<p v-else>
			No topics found.
			<Button @click="onEvent('refresh')" size="sm">
				Load Topics
			</Button>
		</p>
	</div>
</template>
