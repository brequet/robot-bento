

export interface ApiSuiteKeywords {
    setupKeyword?: RobotKeyword;
    teardownKeyword?: RobotKeyword;
}

export type RobotBaseBody =
    | { kw: RobotKeyword }
    | { for: RobotFor }
    | { while: RobotWhile }
    | { group: RobotGroup }
    | { if: RobotIf }
    | { try: RobotTry }
    | { variable: RobotVar }
    | { return: RobotReturn }
    | { continue: RobotContinue }
    | { break: RobotBreak }
    | { message: RobotMessage }

export interface RobotStatus {
    status: string;
    start_time: string;
    end_time: string;
}

export interface RobotBreak {
    status: RobotStatus;
}

export interface RobotContinue {
    status: RobotStatus;
}

export interface RobotForIterVar {
    name: string;
    value?: string;
}

export interface RobotForIter {
    children: RobotBaseBody[];
    vars: RobotForIterVar[];
    status: RobotStatus;
}

export interface RobotFor {
    flavor: string;
    start?: string;
    mode?: string;
    fill?: string;
    iters: RobotForIter[];
    vars: string[];
    values: string[];
    status: RobotStatus;
}

export interface RobotGroup {
    name: string;
    children: RobotBaseBody[];
    status: RobotStatus;
}

export interface RobotIfBranch {
    type_: string;
    condition?: string;
    children: RobotBaseBody[];
    status: RobotStatus;
}

export interface RobotIf {
    branches: RobotIfBranch[];
    status: RobotStatus;
}

export interface RobotMessage {
    timestamp: string;
    level: string;
    value: string;
}

type RobotKeywordType = 'SETUP' | 'TEARDOWN' | undefined;

export interface RobotKeyword {
    name: string;
    owner?: string;
    library?: string;
    type_?: RobotKeywordType;
    msg: RobotMessage[];
    keywords: RobotBaseBody[];
    var: string[];
    args: string[];
    tags: string[];
    doc?: string;
    timeout?: string;
    status?: RobotStatus;
}

export interface RobotReturn {
    value: string[];
    status: RobotStatus;
}

export interface RobotTryBranch {
    type: string;
    patternType?: string;
    assign?: string;
    children: RobotBaseBody[];
    status: RobotStatus;
}

export interface RobotTry {
    branches: RobotTryBranch[];
    status: RobotStatus;
}

export interface RobotVar {
    name: string;
    scope?: string;
    separator?: string;
    message: RobotMessage;
    vars: string[];
    status: RobotStatus;
}

export interface RobotWhileIter {
    children: RobotBaseBody[];
    status: RobotStatus;
}

export interface RobotWhile {
    condition: string[];
    limit: string[];
    onLimit: string[];
    onLimitMessage: string[];
    iters: RobotWhileIter[];
    status: RobotStatus;
}

export type KeywordType =
    | 'RobotKeyword'
    | 'RobotFor'
    | 'RobotWhile'
    | 'RobotGroup'
    | 'RobotIf'
    | 'RobotTry'
    | 'RobotVar'
    | 'RobotReturn'
    | 'RobotContinue'
    | 'RobotBreak'
    | 'RobotMessage';

