<template>
    <div class="hello" @click="foo">
        sort
    </div>
    <user-input @input="rawArray = $event"/>
    <heap :array="sortedArray"/>
</template>

<script setup lang="ts">
import {onBeforeMount, ref, watch} from "vue"
import init, {to_heap} from "../../rust/pkg/"
import Heap from "@/components/Heap.vue"
import UserInput from "@/components/UserInput.vue";

const rawArray = ref<number[]>([])
const sortedArray = ref<number[]>([])

function foo() {
    sortedArray.value = [...to_heap(new Uint8Array(rawArray.value))]
}

watch(() => rawArray.value,
    (array) => {
        if (array.length === 0) {
            sortedArray.value = []
        }
    })
onBeforeMount(init)
</script>
<style scoped lang="scss">
.hello {
  cursor: pointer;
}
</style>
