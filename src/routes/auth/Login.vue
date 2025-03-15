<script setup>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import Swal from "sweetalert2";
import LandingPage from "../../components/LandingPage.vue";

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
            title: response.message,
            text: response.error,
        })
    }

  } catch (error) {
    Swal.fire({
        icon: "error",
        title: error.message,
        text: error.error,
    })
  }
}

</script>

<template>
    <div class="container-fluid">
        <div class="row">
            <div class="col-lg-4 card-form-login">
                <div class="card">
                    <form  @submit.prevent="login">
                        <h5>Login</h5>
                        <div class="form-group mb-3">
                            <label for="email">Email address</label>
                            <input type="email" class="form-control" id="email" v-model="form.email">
                        </div>
                        <div class="form-group mb-3">
                            <label for="password">Password</label>
                            <input type="password" class="form-control" id="password" v-model="form.password">
                        </div>
                        <button type="submit" class="btn btn-success w-100 text-white my-3">Login</button>
                    </form>
                </div>
            </div>
            <div class="col-lg-8 card-landing-page">
                <div class="card">
                    <div class="overlay"></div>
                    <LandingPage/>
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="scss">
    .container-fluid {
        height: 100vh;
        background: url("/img/gambar-3.jpg");
        background-size: cover;

        .row {
            height: 100%;
            justify-content: center;
            align-items: center;
            .card-form-login {
                padding: 6rem 0 6rem 12rem;
                // background-color: salmon;

                form {
                    padding: 1rem;
                    border-radius: 10px;
                    box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.1);
                }
            }

            .card-landing-page {
                padding: 6rem;

                .overlay {
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    z-index: 1;
                }

                .carousel {
                    padding: 1rem;
                }
            }
        }

    }
</style>