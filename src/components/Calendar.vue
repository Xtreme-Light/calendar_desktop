<script setup lang="ts">
import {FullCalendar }from '@fullcalendar/vue3'
import CalendarOptions from '@fullcalendar/core'
import dayGridPlugin from '@fullcalendar/daygrid'
import interactionPlugin from '@fullcalendar/interaction'
import { onMounted, ref } from 'vue'

const handleEventClick = (eventClickInfo: any) => {
    console.log('any event' + eventClickInfo)
}


const handleDateClick = (arg: { dateStr: string; }) =>{
    console.log('date click! ' + arg.dateStr)
}

const calendarOptions  = ref<CalendarOptions>({
    plugins: [dayGridPlugin, interactionPlugin],
    initialView: 'dayGridMonth',
    buttonText: {
        today: '今天',
        month: '月视图',
        week: '周视图',
        day: '日视图',
    },
    locale:'zh',
    dateClick: handleDateClick,
    eventClick: handleEventClick,
});





// 生命周期钩子
onMounted(() => {
    console.log(`init `)
})

</script>


<template>
    <FullCalendar :options="calendarOptions"  ref="fullcalendarref" />
</template>