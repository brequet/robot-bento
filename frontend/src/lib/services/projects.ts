import { API_BASE_URL } from '$lib/config';

import type { ProjectOverview, Project } from '$lib/types/project';

export async function getAllProjects(): Promise<ProjectOverview[]> {
	try {
		const response = await fetch(`${API_BASE_URL}/projects`);

		if (!response.ok) {
			throw new Error(`Failed to fetch projects: ${response.statusText}`);
		}

		return await response.json();
	} catch (error) {
		console.error('Error fetching projects:', error);
		throw error;
	}
}

export async function getProjectById(id: string): Promise<Project | null> {
	try {
		const response = await fetch(`${API_BASE_URL}/projects/${id}`);
		if (!response.ok) throw new Error('Failed to fetch project details');
		return await response.json();
	} catch (error) {
		console.error('Error fetching project:', error);
		return null;
	}
}
