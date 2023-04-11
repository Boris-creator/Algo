<template>
    <svg preserveAspectRatio="xMidYMid meet" :viewBox="`0 0 ${totalWidth} ${totalHeight}`">
        <g>
            <template v-for="(_, i) of parentNodes">
                <line :x1="coordinates[i].x"
                      :y1="coordinates[i].y"
                      :x2="coordinates[i * 2 + 1].x"
                      :y2="coordinates[i * 2 + 1].y"
                      stroke="black"
                      :stroke-width="coordinates[i].r * strokeWidth"
                />
                <line v-if="i * 2 + 2 < array.length"
                      :x1="coordinates[i].x"
                      :y1="coordinates[i].y"
                      :x2="coordinates[i * 2 + 2].x"
                      :y2="coordinates[i * 2 + 2].y"
                      stroke="black"
                      :stroke-width="coordinates[i].r * strokeWidth"
                />
            </template>
        </g>
        <g v-for="(el, i) of array">
            <circle :r="coordinates[i].r"
                    :cx="coordinates[i].x"
                    :cy="coordinates[i].y"
                    :stroke-width="coordinates[i].r * strokeWidth"
                    fill="white"
                    stroke="black"
            />
            <text :x="coordinates[i].x"
                  :y="coordinates[i].y"
                  text-anchor="middle"
                  alignment-baseline="middle"
                  :font-size="coordinates[i].r"
            >
                {{el}}
            </text>
        </g>
    </svg>
</template>
<script setup lang="ts">
import {defineProps, computed} from "vue"

const props = defineProps<{
    array: number[]
}>()
const totalWidth = 100
const totalHeight = 100
const strokeWidth = .2

const parentNodes = computed(() => Array(Math.floor(props.array.length / 2)))
const coordinates = computed(() => {
    const {array} = props
    const rowsCount = Math.floor(Math.log2(array.length)) + 1
    const lastRowIndex = rowsCount - 1
    const baseWidth = 2 ** lastRowIndex + 1
    const baseGapX = totalWidth / (baseWidth - 1)
    const gapStepY = 0.9
    const baseGapY = totalHeight * (gapStepY - 1) / (gapStepY ** rowsCount - 1) * gapStepY
    // totalHeight * (gapStepY - 1) / (gapStepY ** (rowsCount - 1) - 1) would be the base of geometric progression with step = gapStepY and sum = totalHeight. I set one more row to leave some space.

    let marginTop: number

    const points: Record<number, { x: number, y: number, r: number }> = {}
    array.forEach((_, i) => {
        const row = Math.ceil(Math.log2(i + 2)) - 1
        const column = i - 2 ** row + 1
        const gapX = baseGapX * 2 ** (lastRowIndex - row)
        const gapY = baseGapY * gapStepY ** row
        const rowWidth = (2 ** row - 1) * gapX

        const radius = rowsCount === 1 ? totalHeight / 3 : Math.min(gapX / 3, gapY / 2)
        if (i === 0) {
            const height = baseGapY * (gapStepY ** (rowsCount - 1) - 1) / (gapStepY - 1) // sum of geometric progression
            marginTop = (totalHeight - height) / 2
        }
        const marginLeft = (totalWidth - rowWidth) / 2
        points[i] = {
            x: marginLeft + column * gapX,
            y: marginTop + baseGapY * (gapStepY ** row - 1) / (gapStepY - 1),
            r: radius
        }
    })
    return points
})
</script>
<style scoped lang="scss">
svg {
  border: thin solid silver;
    width: 40vw;
}
</style>
