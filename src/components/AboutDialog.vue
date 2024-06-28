<script lang="ts" setup>
import { arch } from '@tauri-apps/plugin-os'
import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app'
import { open } from '@tauri-apps/plugin-shell'
import { HomeIcon, RefreshCw, XIcon } from 'lucide-vue-next'
import { Icons } from "./Icons"
import { DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter, DialogClose} from "./ui/dialog"
import { onMounted, ref } from 'vue'
import { Button } from './ui/button'

const version = ref()
const arc = ref()
const name = ref()
const tauriVersion = ref()
const updateText = ref('')
function setUpdateText(text: string) {
  updateText.value = text
}

onMounted(async () => {
  version.value = await getVersion()
  arc.value = await arch()
  name.value = await getName()
  tauriVersion.value = await getTauriVersion()
})
</script>

<template>
  <DialogContent class="overflow-clip pb-2">
    <DialogHeader class="flex items-center text-center">
      <div class="rounded-full bg-background p-[6px] text-slate-600 drop-shadow-none transition duration-1000 hover:text-slate-800 hover:drop-shadow-[0_0px_10px_rgba(0,10,50,0.50)] dark:hover:text-slate-400 ">
        <Icons.logo class="h-12 w-12" />
      </div>
      <DialogTitle class="flex flex-col items-center gap-2 pt-2">
        Tauri UI ({{ name }})
        <span class="flex gap-1 font-mono text-xs font-medium">
          Version {{ version }} ({{ arc }})
          <span class="font-sans font-medium text-gray-400">
            <span class="cursor-pointer text-blue-500" @click=" open('github.com/agmmnn/tauri-ui/releases/tag/v0.2.0')">release notes</span>
          </span>
        </span>
      </DialogTitle>
      <DialogDescription class="text-foreground">
        App description.
      </DialogDescription>
      <span class="text-xs text-gray-400">{{ updateText }}</span>
      <DialogDescription class="flex flex-row" />
    </DialogHeader>
    <span class="font-mono text-xs font-medium text-gray-400">
      Tauri version: {{ tauriVersion }}
    </span>
    <DialogFooter class="flex flex-row items-center border-t pt-2 text-slate-400 ">
      <div class="mr-auto flex flex-row gap-2">
        <HomeIcon class="h-5 w-5 cursor-pointer transition hover:text-slate-300" @click="open('https://github.com/agmmnn/tauri-ui')" />
        <XIcon class="h-5 w-5 cursor-pointer transition hover:text-slate-300 " @click=" open('https://github.com/agmmnn/tauri-ui')" />
      </div>
      <Button type="submit" variant="outline" class="h-7 gap-1" @click="setUpdateText('You have the latest version.')">
        <RefreshCw />
      </Button>
      <DialogClose as-child>
        <Button type="button" variant="ghost" class="h-7">
          Close
        </Button>
      </DialogClose>
    </DialogFooter>
  </DialogContent>
</template>
