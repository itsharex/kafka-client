<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Calendar } from '@/components/ui/calendar';
import { CommandEmpty, CommandGroup, CommandItem, CommandList } from '@/components/ui/command';
import { Input } from '@/components/ui/input';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { TagsInput, TagsInputInput, TagsInputItem, TagsInputItemDelete, TagsInputItemText } from '@/components/ui/tags-input';
import { GroupOffset } from '@/lib/kafka';
import { cn, getLang } from '@/lib/utils';
import { CalendarIcon } from 'lucide-vue-next';
import { type DateValue, getLocalTimeZone, DateFormatter, today } from '@internationalized/date'
import { ComboboxAnchor, ComboboxInput, ComboboxPortal, ComboboxRoot } from 'radix-vue';
import { computed, ref, onUpdated } from 'vue';
import { Label } from '@/components/ui/label';
const props = defineProps<{ topicList: string[]}>()
onUpdated(() => console.log("Topics updated", props.topicList))
const emit = defineEmits<{
  create: [data: {groupId: string, topics: string[], offset: GroupOffset}]
}>()
const groupId = ref("");
const df = new DateFormatter(getLang(), {
  dateStyle: 'long',
});

const isTopicComboboxOpen = ref(false);
const topicSearchTerm = ref("");
const filteredTopicsList = computed(() => {
  const val = props.topicList.filter(topic => topic.startsWith(topicSearchTerm.value));
  console.log(val);
  return val;
});
const topicNames = ref<string[]>([]);
const offsetType = ref<GroupOffset["type"]>("End");
const offsetTimestamp = ref<DateValue>(today(getLocalTimeZone()));
const onSubmit = () => {
  const offset: GroupOffset = offsetType.value === "Offset" || offsetType.value === "Tail" 
    ? {type: offsetType.value, content: offsetTimestamp.value.toDate(getLocalTimeZone()).getTime()*1000} // Kafka Timestamps are in Nanoseconds
    : {type: offsetType.value};


  emit("create", {groupId: groupId.value, topics: topicNames.value, offset});
  isTopicComboboxOpen.value = false;
  offsetType.value = "End";
  offsetTimestamp.value = today(getLocalTimeZone());
}
</script>
<template>
  <form class="grid gap-5 py-4" @submit.prevent.stop="onSubmit">
    <div class="grid gap-2">
      <Label for="group_id">
        Group ID
      </Label>
      <Input id="group_id" v-model="groupId" />
    </div>
    <div class="grid gap-1">
      <Label for="topics_list">
        Topics
      </Label>
      <TagsInput class="px-0 gap-0" :model-value="topicNames">
        <div class="flex gap-2 flex-wrap items-center px-3">
          <TagsInputItem v-for="item in topicNames" :key="item" :value="item">
            <TagsInputItemText />
            <TagsInputItemDelete />
          </TagsInputItem>
        </div>

        <ComboboxRoot v-model="topicNames" v-model:open="isTopicComboboxOpen" v-model:searchTerm="topicSearchTerm" class="w-full">
          <ComboboxAnchor as-child>
            <ComboboxInput placeholder="Topics..." as-child @blur="isTopicComboboxOpen = false" @focus="isTopicComboboxOpen = true">
              <TagsInputInput class="w-full px-3" :class="topicNames.length > 0 ? 'mt-2' : ''" @keydown.enter.prevent />
            </ComboboxInput>
          </ComboboxAnchor>

          <ComboboxPortal to="#app">
            <CommandList :style="{zIndex: '50'}"
              position="popper"
              class="w-[--radix-popper-anchor-width] rounded-md mt-2 border bg-popover text-popover-foreground shadow-md outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2"
            >
              <CommandEmpty></CommandEmpty>
              <CommandGroup>
                <CommandItem
                  v-for="topic in filteredTopicsList" :key="topic" :value="topic"
                  @select.prevent="(ev) => {
                    if (typeof ev.detail.value === 'string') {
                      topicSearchTerm = ''
                      topicNames.push(ev.detail.value)
                    }

                    if (filteredTopicsList.length === 0) {
                      isTopicComboboxOpen = false
                    }
                  }"
                >
                  {{ topic }}
                </CommandItem>
              </CommandGroup>
            </CommandList>
          </ComboboxPortal>
        </ComboboxRoot>
      </TagsInput>
    </div>
    <div class="grid gap-2">
      <Label for="group_id">
        Offset
      </Label>
      <div class="flex space-x-2">
        <Select v-model:model-value="offsetType">
          <SelectTrigger class="w-[180px]">
            <SelectValue placeholder="Choose Offset Type" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectItem value="End">
                Latest
              </SelectItem>
              <SelectItem value="Beginning">
                Earliest
              </SelectItem>
              <SelectItem value="Offset">
                By Timestamp
              </SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
        <Popover v-if="offsetType === 'Offset'">
          <PopoverTrigger as-child>
            <Button variant="outline"
              :class="cn(
                'w-[280px] justify-start text-left font-normal',
                !offsetTimestamp && 'text-muted-foreground',
              )"
            >
              <CalendarIcon class="mr-2 h-4 w-4" />
              {{ offsetTimestamp ? df.format(offsetTimestamp.toDate(getLocalTimeZone())) : "Pick a date" }}
            </Button>
          </PopoverTrigger>
          <PopoverContent class="w-auto p-0">
            <Calendar v-model="offsetTimestamp" initial-focus />
          </PopoverContent>
        </Popover>
      </div>
    </div>
  </form>
</template>
