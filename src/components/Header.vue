<script setup>
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";

const router = useRouter();

async function logout() {
    await invoke("logout");
    router.push("/login");
}
    
</script>

<template>
    <h1>Header</h1>
    <button class="btn btn-danger" @click="logout">Logout</button>
</template>