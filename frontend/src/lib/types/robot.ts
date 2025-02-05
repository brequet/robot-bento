export interface TestRun {
	id: number;
	rpa: boolean;
	generator: string;
	generated_date: string;
	schema_version: string;
	imported_date: string;
	suites: Suite[];
	statistics: Stat[];
	errors: ErrorLog[];
	sha1: string;
	app_name: string;
	app_version: string;
}

export interface Suite {
	id: number;
	name: string;
	source?: string;
	status: string;
	start_time: string;
	end_time: string;
	doc?: string;
	identifier: string;
	setup_keyword?: any;
	suites: Suite[];
	tests: Test[];
	teardown_keyword?: string;
}

export interface Test {
	id: number;
	name: string;
	line: number;
	identifier: string;
	tags: string[];
	status: string;
	start_time: string;
	end_time: string;
	doc?: string;
	timeout?: string;
	keywords: any[];
}

export interface Stat {
	id: number;
	stat_type: 'Total' | 'Tag' | 'suite';
	pass_count: number;
	fail_count: number;
	skip_count: number;
	identifier?: string;
	name?: string;
	text: string;
}

export interface ErrorLog {
	id: number;
	timestamp: string;
	level: string;
	content: string;
}
