import { API_BASE_URL } from '$lib/config';
import type { ApiSuite, ApiTest, TestRunResponse } from '$lib/types/generated';
import type { ApiSuiteKeywords, RobotBaseBody } from '$lib/types/robot';


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

export async function getSuiteKeywords(suiteId: number): Promise<ApiSuiteKeywords | null> {
	try {
		const response = await fetch(`${ROBOT_BASE_API}/suites/${suiteId}/keywords`);
		if (!response.ok) throw new Error('Failed to fetch suites keywords');
		return await response.json();
	} catch (error) {
		console.error('Error fetching suites keywords:', error);
		return null;
	}
}

export async function getTestKeywords(testId: number): Promise<RobotBaseBody[] | null> {
	try {
		const response = await fetch(`${ROBOT_BASE_API}/tests/${testId}/keywords`);
		if (!response.ok) throw new Error('Failed to fetch test keywords');
		return await response.json();
	} catch (error) {
		console.error('Error fetching test keywords:', error);
		return null;
	}
}

export function getFailedTestIdentifiers(suites: ApiSuite[]): string[] {
	const failedTestIds: string[] = [];
	for (const suite of suites) {
		for (const test of suite.tests) {
			if (test.status === 'FAIL') {
				failedTestIds.push(test.identifier);
			}
		}
		failedTestIds.push(...getFailedTestIdentifiers(suite.suites));
	}
	return failedTestIds;
}

export function findTestByIdentifier(suites: ApiSuite[], testIdentifier: string): ApiTest | null {
	for (const suite of suites) {
		for (const test of suite.tests) {
			if (testIdentifier === test.identifier) {
				return test;
			}
		}
		const foundTest = findTestByIdentifier(suite.suites, testIdentifier);
		if (foundTest) {
			return foundTest;
		}
	}
	return null;
}

export function findSuiteByIdentifier(suites: ApiSuite[], suiteIdentifier: string): ApiSuite | null {
	for (const suite of suites) {
		if (suiteIdentifier === suite.identifier) {
			return suite;
		}
		const foundSuite = findSuiteByIdentifier(suite.suites, suiteIdentifier);
		if (foundSuite) {
			return foundSuite;
		}
	}
	return null;
}
