<template>
    <div class="new-position">

    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
    setup () {
        

        return {}
    }
})
</script>

<style scoped>
.new-position {
    display: flex;
    flex-direction: column;
    padding: 1rem;
    align-items: center;
    position: absolute;
    left: 0;
    top: 0;
    right: 0;
    bottom: 0;
    margin: 2rem;
    border: 1px solid #ccc;
}

@media (prefers-color-scheme: dark) {
    .new-position {
        color: rgba(255, 255, 255, 0.87);
        background-color: #242424;
    }
}
@media (prefers-color-scheme: light) {
    .new-position {
        color: #213547;
        background-color: #ffffff;
    }
}
</style>