export function formatDate(dateString: string): string {
    const date = new Date(dateString);
    return formatStandardDate(date);
}

export function formatRobotDate(dateString: string): string {
    const date = parseRobotDate(dateString);
    return formatStandardDate(date);
}

const formatStandardDate = (date: Date): string => {
    const months = [
        'Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun',
        'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'
    ];
    const month = months[date.getMonth()];
    const day = date.getDate();
    const year = date.getFullYear();
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    return `${month} ${day}, ${year} at ${hours}:${minutes}`;
};

const parseRobotDate = (dateString: string): Date => {
    const [datePart, timePart] = dateString.split(' ');
    const year = parseInt(datePart.substring(0, 4), 10);
    const month = parseInt(datePart.substring(4, 6), 10) - 1; // Months are 0-based in JavaScript Date
    const day = parseInt(datePart.substring(6, 8), 10);
    const [hours, minutes, seconds] = timePart.split(':').map(part => parseInt(part, 10));
    const milliseconds = parseInt(timePart.split('.')[1], 10);
    return new Date(year, month, day, hours, minutes, seconds, milliseconds);
};


export const formatElapsedTime = (start: string, end: string): string => {
    const diff = new Date(end).getTime() - new Date(start).getTime();
    return formatTimeDiff(diff);
};

export const formatRobotElapsedTime = (start: string, end: string, useNanoSeconds: boolean = true): string => {
    const startDate = parseRobotDate(start);
    const endDate = parseRobotDate(end);
    const diff = endDate.getTime() - startDate.getTime();
    return formatTimeDiff(diff, useNanoSeconds);
};

const formatTimeDiff = (diff: number, useNanoSeconds: boolean = false): string => {
    const hours = Math.floor(diff / 3600000);
    const minutes = Math.floor((diff % 3600000) / 60000);
    const seconds = Math.floor((diff % 60000) / 1000);
    const milliseconds = diff % 1000;

    const formattedResult = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`

    if (useNanoSeconds) {
        return `${formattedResult}.${milliseconds.toString().padStart(3, '0')}`;
    }

    return formattedResult;
};
