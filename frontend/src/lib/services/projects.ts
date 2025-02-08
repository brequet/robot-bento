import { API_BASE_URL } from '$lib/config';

// TODO: project should be generated from api model
import type { Project, ProjectOverviewResponse } from '$lib/types/generated';

const PROJECTS_BASE_API = `${API_BASE_URL}/projects`;

export async function getAllProjects(): Promise<ProjectOverviewResponse[]> {
	try {
		const response = await fetch(`${PROJECTS_BASE_API}/overview`);

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
		const response = await fetch(`${PROJECTS_BASE_API}/${id}`);
		if (!response.ok) throw new Error('Failed to fetch project details');
		return await response.json();
	} catch (error) {
		console.error('Error fetching project:', error);
		return null;
	}
}
