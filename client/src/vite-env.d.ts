/// <reference types="svelte" />
/// <reference types="vite/client" />

declare namespace svelteHTML {
    interface HTMLAttributes<T> {
        'on:view'?: (event: CustomEvent) => any;
        'on:exit'?: (event: CustomEvent) => any;
    }
}