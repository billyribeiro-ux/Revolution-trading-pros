import { addCollection, type IconifyJSON } from '@iconify/svelte';
import heroicons from '@iconify-json/heroicons/icons.json';
import ph from '@iconify-json/ph/icons.json';

addCollection(ph as IconifyJSON);
addCollection(heroicons as IconifyJSON);
