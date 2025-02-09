import { API_BASE_URL } from '$lib/config';
import type { TestRunResponse } from '$lib/types/generated';


const ROBOT_BASE_API = `${API_BASE_URL}/robot`;

export async function getTestRunById(id: number): Promise<TestRunResponse | null> {
	try {
		const response = await fetch(`${ROBOT_BASE_API}/test-runs/${id}`);
		if (!response.ok) throw new Error('Failed to fetch robot test run');
		return await response.json();
	} catch (error) {
		console.error('Error fetching robot test run:', error);
		return null;
	}
}
