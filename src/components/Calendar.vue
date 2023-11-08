<script setup lang="ts">
import FullCalendar from '@fullcalendar/vue3'
import {CalendarOptions,EventClickArg,EventHoveringArg} from '@fullcalendar/core'
import dayGridPlugin from '@fullcalendar/daygrid'
import interactionPlugin,{DateClickArg} from '@fullcalendar/interaction'
import { onMounted, ref } from 'vue'

const handleEventClick = (eventClickInfo: EventClickArg) => {
    console.log('any event' + eventClickInfo)
}


const handleDateClick = (arg:DateClickArg) =>{
    console.log('date click! ' + arg.dateStr)

}
const handleEventMouseEnter = (arg: EventHoveringArg) =>{
    console.log('鼠标移入'+ arg)
}

const handleEventMouseLeave = (arg: EventHoveringArg) =>{
    console.log('鼠标移出'+ arg)
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
    eventMouseEnter:handleEventMouseEnter,
    eventMouseLeave: handleEventMouseLeave,
});





// 生命周期钩子
onMounted(() => {
    console.log(`init `)
})

</script>


<template>
    <FullCalendar :options="calendarOptions"  ref="fullcalendarref" />
</template>