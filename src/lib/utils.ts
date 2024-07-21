import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function getLang() {
  if (navigator.languages != undefined) return navigator.languages[0];
  return navigator.language;
}

export function jsonText<T>(obj: string): T|null {
  try {
    return JSON.parse(obj) as T;
  } catch(error) {
    return null
  }
}