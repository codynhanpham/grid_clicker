import type { MenubarExperimentInfo } from './menubar.svelte';

function menuOpenExperiment(MenubarExperimentInfo: MenubarExperimentInfo): undefined {
	console.log('menuOpenExperiment\n', MenubarExperimentInfo);
}

export { menuOpenExperiment };
