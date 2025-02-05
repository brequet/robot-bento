import type { TestRun } from './robot';

export interface ProjectOverview {
	id: number;
	name: string;
	test_run_count: number;
	last_test_run?: ProjectTestRun | null;
}

export interface ProjectTestRun {
	last_test_run_date: string;
	total_tests: number;
	passed_tests: number;
	failed_tests: number;
	skipped_tests: number;
}

export interface Project {
	id: number;
	name: string;
	test_run_count: number;
	test_runs: TestRun[];
}
