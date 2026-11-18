import type { L } from "./types";

export async function global<T>(name: string): Promise<T> {
    let v = (window as any)[name];
    while (!v) {
        await wait(100);
        v = (window as any)[name];
    }
    return v;
}

export const setup = () => global<typeof L>("L");

export function wait(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
}