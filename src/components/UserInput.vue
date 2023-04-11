<template>
    <svg ref="area" preserveAspectRatio="xMidYMid meet" :viewBox="`0 0 ${totalWidth} ${totalHeight}`">
        <rect v-for="(y, x) in points"
              :x="x * totalWidth / stepsX"
              :y="totalHeight - y * totalHeight / stepsY"
              :width="totalWidth / stepsX"
              :height="y * totalHeight / stepsY"
        />
    </svg>
</template>
<script setup lang="ts">
import {ref, computed, watch, defineEmits} from "vue"
import {useMousePressed, useMouseInElement} from "@vueuse/core"

type Point = { x: number, y: number }

const emit = defineEmits<{
    (e: "input", value: number[]): void
}>()

const area = ref<HTMLElement | null>(null)

const totalWidth = 300
const totalHeight = 100
const stepsX = 90
const stepsY = 30

const {pressed} = useMousePressed({target: area})
const {elementX, elementY, isOutside} = useMouseInElement(area, {handleOutside: false})
const points = ref<Record<number, number>>({})

const currentPoint = computed<Point | null>(() => {
    if (!pressed.value || isOutside.value) {
        return null
    }
    const width = area.value?.getBoundingClientRect().width ?? totalWidth
    const height = area.value?.getBoundingClientRect().height ?? totalHeight
    return {
        x: Math.floor(elementX.value / (width / stepsX)),
        y: Math.floor((height - elementY.value) / (height / stepsY))
    }
})

function clearPoints() {
    points.value = {}
    emit("input", [])
}
function interpolate(start: Point, end: Point) {
    [start, end] = [start, end].sort(({x: x1}, {x: x2}) => x1 - x2)
    const average = (start.y + end.y) / 2
    for(let index = start.x + 1; index < end.x; index++) {
        points.value[index] = average
    }
}

const extractValues = (userInput: typeof points.value) => Object.entries(userInput).sort(([x1], [x2]) => +x1 - +x2).map(([_, y]) => y)

watch(() => currentPoint.value, (point, prevPoint) => {
    if (point === null) {
        emit("input", extractValues(points.value))
        return
    }
    if (prevPoint === null) {
        clearPoints()
    }
    /*if (point.x !== prevPoint?.x && typeof points.value[point.x] !== "undefined") {
        clearPoints()
    }*/
    points.value[point.x] = point.y
    if(prevPoint !== null) {
        interpolate(prevPoint, point)
    }
}, {deep: true})
</script>
<style scoped lang="scss">
svg {
  display: block;
  border: thin solid silver;
  width: 40vw;

  rect {
    fill: cornflowerblue;
  }
}
</style>
