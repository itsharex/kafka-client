<script setup lang="ts">
import { Fzf } from 'fzf';
import { computed, ref } from 'vue';
import { RadioGroup, RadioGroupOption } from '@headlessui/vue';
import { ArrowPathIcon, } from '@heroicons/vue/16/solid';

import Input from '@/components/ui/input/Input.vue';
import Button from '@/components/ui/button/Button.vue';

import { ConsumerGroup } from '@/lib/kafka';

const props = defineProps<{ groups: ConsumerGroup[], error: string }>();
const onEvent = defineEmits(['refresh']);
const selectedGroupModel = defineModel<ConsumerGroup | undefined>('selectedGroup')
const groupsSearchIndex = computed(() => new Fzf(props.groups, { selector: (d) => d.name, sort: true }));
const search = ref("");
const filteredGroupList = computed(() =>
	groupsSearchIndex.value
		.find(search.value)
		.sort((a, b) => b.score - a.score)
		.map((e) => e.item)
); 

</script>
<template>
	<div
		class="flex items-center justify-between leading-none py-1 px-2 border-t border-muted">
		<h4 class="uppercase text-xs tracking-wide font-bold">Topics</h4>
		<div class="space-x-2">
			<Button variant="outline" size="xs" @click="onEvent('refresh')">
				<ArrowPathIcon class="block w-4 h-4" />
			</Button>
		</div>
	</div>
	<div class="px-2 py-2 border-t border-muted">
		<Input id="search-input" class="w-full form-input py-2 rounded" v-model="search"
			placeholder="Search Groups..." />
		<p v-if="props.error" class="-mx-2 px-2 py-1 leading-tight bg-error-100 text-error-800 my-2"
			v-text="props.error"></p>
	</div>
	<div class="px-2">
		<RadioGroup v-if="filteredGroupList.length > 0" v-model="selectedGroupModel">
			<RadioGroupOption v-for="group of filteredGroupList" :key="group.name"
				class="-mx-2 px-4 py-1 hover:bg-neutral-100 dark:hover:bg-neutral-800 ui-checked:bg-primary ui-checked:text-white cursor-pointer"
				:value="group">
				<code class="text-xs leading-none font-bold" v-text="group.name"></code>
				<p class="text-opacity-25 font-medium text-xs uppercase">
					Members: {{ group.members.length }}
				</p>
			</RadioGroupOption>
		</RadioGroup>

		<p v-else-if="groups.length > 0">
			No Groups Match the search keywords. <Button variant="link" class="p-0 h-auto" @click.prevent="search = ''">Clear</Button> search keyword.
		</p>
		<p v-else>
			No groups found.
			<Button @click="onEvent('refresh')" size="sm">
				Refresh Groups
			</Button>
		</p>
	</div>
</template>
