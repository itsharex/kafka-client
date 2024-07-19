<script setup lang="ts">
import { Fzf } from 'fzf';
import { computed, onMounted, ref } from 'vue';
import { ArrowPathIcon, } from '@heroicons/vue/16/solid';
import Button from '@/components/ui/button/Button.vue';

import { ConsumerGroup, GroupOffset, createConsumerGroup, deleteConsumerGroup, getClusterMetadata } from '@/lib/kafka';
import { useToast } from '@/components/ui/toast';
import { PlusIcon, TrashIcon } from 'lucide-vue-next';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import NewConsumerGroupForm from '@/components/ConsumerGroups/NewConsumerGroupForm.vue';
import { Command, CommandEmpty, CommandItem, CommandList } from '@/components/ui/command';
import CommandInput from '@/components/ui/command/CommandInput.vue';
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from '@/components/ui/alert-dialog';

const {toast} = useToast();
const topicsList = ref<string[]>(["test-topic"]);
function fetchTopics() {
  getClusterMetadata()
    .then((data) => {
      topicsList.value = data.topics.map(t => t.name);
    })
    .catch((err) => {
      toast({title: "Error", description: err});
    });
}
onMounted(() => fetchTopics());

const props = defineProps<{ groups: ConsumerGroup[], error: string }>();
const onEvent = defineEmits(['refresh']);
const isCreateGroupDialogOpen = ref(false);
const selectedGroupModel = defineModel<ConsumerGroup | undefined>('selectedGroup')
const groupsSearchIndex = computed(() => new Fzf(props.groups, { selector: (d) => d.name, sort: true }));
const filterGroupList = (searchTerm: string) =>
	groupsSearchIndex.value
		.find(searchTerm)
		.sort((a, b) => b.score - a.score)
		.map((e) => e.item);

const deleteGroup = (group: string) => {
	deleteConsumerGroup(group)
		.then((g) => toast({title: "Success!", description: `Consumer group '${g}' deleted successfully!'`}))
		.then(() => onEvent("refresh"))
		.catch(err => toast({title: "Error!", variant: "destructive", description: err instanceof Error ? err.message : ""+err}))
}

const createNewConsumerGroup = (data: {groupId: string, topics: string[], offset: GroupOffset}) => {
  createConsumerGroup(data.groupId, data.topics, data.offset)
    .then(() => {
		onEvent("refresh");
		isCreateGroupDialogOpen.value = false;
	})
    .catch((err) => toast({
      title: "Error",
      description: err
    }));
}
</script>
<template>
	<div
		class="flex items-center justify-between leading-none py-1 px-2 border-t border-muted">
		<h4 class="uppercase text-xs tracking-wide font-bold">Consumer Groups</h4>
		<div class="space-x-2">
			<Dialog v-model:open="isCreateGroupDialogOpen">
				<DialogTrigger as-child>
					<Button variant="outline" size="xs">
						<PlusIcon class="block w-4 h-4" />
					</Button>
				</DialogTrigger>
				<DialogContent>
					<DialogHeader>
						<DialogTitle>New Consumer Group</DialogTitle>
						<DialogDescription>Commit new consumer group offsets to the broker</DialogDescription>
					</DialogHeader>
					<NewConsumerGroupForm id="create-consumer-group-form" :topic-list="topicsList" @create="createNewConsumerGroup"></NewConsumerGroupForm>
					<DialogFooter>
						<Button type="submit" form="create-consumer-group-form">Create Group</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
			<Button variant="outline" size="xs" @click="onEvent('refresh')">
				<ArrowPathIcon class="block w-4 h-4" />
			</Button>
		</div>
	</div>
	<div class="px-2">
		<Command v-model="selectedGroupModel" :filter-function="(_, search) => filterGroupList(search)">
			<CommandInput id="search-input" placeholder="Search Groups..." />
			<p v-if="props.error" class="-mx-2 px-2 py-1 leading-tight bg-error-100 text-error-800 my-2"
				v-text="props.error"></p>
			<CommandList>
				<CommandEmpty>
					<p v-if="groups.length > 0">
						No Groups Match the search keywords. Try clearing search keyword.
					</p>
					<p v-else>
						No groups found.
						<Button variant="link" @click="onEvent('refresh')" size="xs">
							Refresh Groups
						</Button>
					</p>
				</CommandEmpty>
				<CommandItem v-for="group of groups" :key="group.name" :value="group" class="flex items-center space-x-2">
					<p class="flex-1" v-text="`${group.name} (${group.members.length})`"></p>
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
									This action cannot be undone. This will permanently delete all the committed topic offsets for this group.
								</AlertDialogDescription>
							</AlertDialogHeader>
							<AlertDialogFooter>
								<AlertDialogCancel>No way, Take me back!</AlertDialogCancel>
								<AlertDialogAction @click="() => deleteGroup(group.name)">Yes, Absolutely!</AlertDialogAction>
							</AlertDialogFooter>
						</AlertDialogContent>
					</AlertDialog>
				</CommandItem>
			</CommandList>
		</Command>
	</div>
</template>
