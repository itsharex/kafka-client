import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function getLang() {
  if (navigator.languages != undefined) return navigator.languages[0];
  return navigator.language;
}