<script>
import { ref } from 'vue';

const slide = ref(0);
const sliding = ref(null);

const slideInterval = ref(5000)

const onSlideStart = () => {
  sliding.value = true;
};

const onSlideEnd = () => {
  sliding.value = false;
};
</script>

<template>
    <BCarousel :interval="slideInterval" indicators ride="carousel">
        <BCarouselSlide style="background-color: transparent;">
            <h1>First slide</h1>
            <p>Some more detailed description or whatever content.</p>
        </BCarouselSlide>
        <BCarouselSlide style="background-color: transparent;">
            <h1>First slide</h1>
            <p>Some more detailed description or whatever content.</p>
        </BCarouselSlide >
        <BCarouselSlide style="background-color: transparent;">
            <h1>First slide</h1>
            <p>Some more detailed description or whatever content.</p> 
        </BCarouselSlide>
    </BCarousel>
</template>

<style scoped lang="scss">

    .carousel-item {
        position: relative;
        height: 50vh;
        min-height: 50vh;
    }

    h1 {
        z-index: 2;
        color: #222;
    }
    p {
        z-index: 2;
        color: #222;
    }
</style>