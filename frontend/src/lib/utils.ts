import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export type WithElementRef<T> = T & {
	ref?: HTMLElement | null;
};

export type WithoutChildren<T> = Omit<T, "children">;
export type WithoutChild<T> = Omit<T, "child">;
export type WithoutChildrenOrChild<T> = Omit<T, "children" | "child">;

export function getTableKeys(data: any): string[] {
	if (!data) return [];
	const items = Array.isArray(data) ? data : [data];
	const keys = new Set<string>();
	items.forEach((item) => {
		if (item && typeof item === 'object') {
			Object.keys(item).forEach((k) => keys.add(k));
		}
	});
	return Array.from(keys);
}

export function getSchema(data: any): Array<{ key: string; type: string }> {
	if (!data) return [];
	const items = Array.isArray(data) ? data : [data];
	const schemaMap = new Map<string, string>();
	items.forEach((item) => {
		if (item && typeof item === 'object') {
			Object.entries(item).forEach(([k, v]) => {
				const type = Array.isArray(v) ? 'array' : typeof v;
				schemaMap.set(k, type);
			});
		}
	});
	return Array.from(schemaMap.entries()).map(([key, type]) => ({ key, type }));
}

export function downloadJSON(data: unknown, filename: string) {
	const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = filename;
	a.click();
	URL.revokeObjectURL(url);
}