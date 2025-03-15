<script setup>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import Swal from "sweetalert2";

const form = ref({});
const router = useRouter();

async function login() {
  try {
    const response = await invoke("login", { payload: form.value });
    if (response.result) {
        router.push("/");
    } else {
        Swal.fire({
            icon: "error",
            title: "Oops...",
            text: response.message,
        })
    }
    console.log(response)

  } catch (error) {
    Swal.fire({
        icon: "error",
        title: error.message,
        text: error.error,
    })
    console.log(error)
  }
}

</script>

<template>
    <div class="container">
        <div class="row">
            <div class="col-md-6">
                <div class="card">
                    <h5>Login</h5>
                    <form  @submit.prevent="login">
                        <div class="form-group">
                            <label for="email">Email address</label>
                            <input type="email" class="form-control" id="email" v-model="form.email">
                        </div>
                        <div class="form-group">
                            <label for="password">Password</label>
                            <input type="password" class="form-control" id="password" v-model="form.password">
                        </div>
                        <button type="submit" class="btn btn-primary w-100 text-white">Login</button>
                    </form>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card">
                    <h1>Landing Page</h1>
                </div>
            </div>
        </div>
    </div>
</template>