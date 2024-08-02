import {writable} from 'svelte/store';
import type { User } from '$lib/types/user';
import { cookieDelete, cookieGet } from '$lib/cookie';
import { unixTime } from '$lib/time';

function createUser() {
    const token = cookieGet('auth-token');
    let user = token ? JSON.parse(atob(token.split('.')[1])) : null;

    if (user && user.exp && user.exp < unixTime()) {
        cookieDelete('auth-token');
        user = null;
    }

    const {subscribe, set } = writable(user as User | null);

    function login(user: User) {
        set(user);
    }

    async function logout() {
        // clear user store
        set(null);

        // fetch logout endpoint to clear cookie
        cookieDelete('auth-token');
    }

    return {
        subscribe,
        login,
        logout,
    }
}

export const user = createUser();