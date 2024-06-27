<script lang="ts" setup>
import { Moon, Sun, SunMoon } from "lucide-vue-next"
import { useTheme } from "@/composables"
import type { Theme } from "@/composables/useTheme"
import { MenubarContent, MenubarMenu, MenubarRadioGroup, MenubarRadioItem, MenubarTrigger } from './ui/menubar';

const { theme, setTheme } = useTheme()

interface ThemeItem {
  label: string
  value: Theme
  icon: typeof Moon | typeof Sun | typeof SunMoon
}
const themeList: Array<ThemeItem> = [
  {
    label: 'Light',
    value: 'light',
    icon: Sun,
  },
  {
    label: 'Dark',
    value: 'dark',
    icon: Moon,
  },
  {
    label: 'System',
    value: 'auto',
    icon: SunMoon,
  },
]
</script>

<template>
  <MenubarMenu>
    <MenubarTrigger>Theme</MenubarTrigger>
    <MenubarContent>
      <MenubarRadioGroup :model-value="theme">
        <MenubarRadioItem v-for="{ label, value, icon } in themeList" :key="value" :value @click="setTheme(value)">
          <component :is="icon" class="mr-2 h-4 w-4" />
          <span>{{ label }}</span>
        </MenubarRadioItem>
      </MenubarRadioGroup>
    </MenubarContent>
  </MenubarMenu>
</template>
