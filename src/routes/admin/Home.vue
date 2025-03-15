<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const user = ref({
    result: false,
    message: "",
    data: null,
    error: "",
});

async function getUserInfo() {
    try {
      const response = await invoke("get_user_info");
      user.value = response;
    } catch (error) {
      user.value = error;
    }
}

onMounted(() => {
    getUserInfo();
});
</script>

<template>
    <h1>Login</h1>
    <p>{{ user.message }}</p>
    <p>{{ user.data?.email }}</p>
</template>