import type { TestRun } from './robot';

export interface ProjectOverviewResponse {
	id: number;
	name: string;
	testRunCount: number;
	lastTestRunSummary?: TestRunSummary | null;
}

export interface TestRunSummary {
	lastTestRunDate: string;
	totalTests: number;
	passedTests: number;
	failedTests: number;
	skippedTests: number;
}

export interface Project {
	id: number;
	name: string;
	test_run_count: number;
	test_runs: TestRun[];
}
