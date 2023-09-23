// Server
import { error } from '@sveltejs/kit';
import type Post from '$lib/Post';

export async function load({ params, fetch }: { params: { id: string }, fetch: (url: string) => Promise<Response> }) {
    // fixed it, now it works on any domain
    const res = await fetch(`/posts/${params.id}.json`); // u can use .json so it has title, date, description, thumbnail, etc
    if (res.status === 404) {
        throw error(404, "Post not found");

    }
    let item = await res.json() as Post;
    if (item.author === "Lynix") {
        item.author = "Lynix (who else did you expect, this is Lynix.ca lol)";
    }

    return item;
}