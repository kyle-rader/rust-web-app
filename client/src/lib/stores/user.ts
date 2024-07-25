import {writable} from 'svelte/store';
import type { User } from '$lib/types/user';
import { cookieDelete, cookieGet } from '$lib/cookie';

function createUser() {
    const token = cookieGet('auth-token');
    console.log('token', token);
    const user = token ? JSON.parse(atob(token.split('.')[1])) : null;
    console.log('user', user);

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