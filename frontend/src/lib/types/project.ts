import type { TestRun } from './robot';

export interface Project {
	id: number;
	name: string;
	test_run_count: number;
	test_runs: TestRun[];
}
