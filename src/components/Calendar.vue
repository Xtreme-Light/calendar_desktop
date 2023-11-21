<script setup lang="ts">
// https://blog.csdn.net/lfwoman/article/details/120177637
// import FullCalendar from '@fullcalendar/vue3'
import { Calendar, EventClickArg, EventHoveringArg, EventInput, EventSourceFunc } from '@fullcalendar/core'
import dayGridPlugin from '@fullcalendar/daygrid'
import interactionPlugin, { DateClickArg } from '@fullcalendar/interaction'
import { onMounted, watch } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

import moment from 'moment';

const handleEventClick = (eventClickInfo: EventClickArg) => {
    console.log('any event' + JSON.stringify(eventClickInfo))
}


const handleDateClick = (arg: DateClickArg) => {
    console.log('date click! ' + JSON.stringify(arg))

}
const handleEventMouseEnter = (arg: EventHoveringArg) => {
    console.log('鼠标移入' + JSON.stringify(arg))
}

const handleEventMouseLeave = (arg: EventHoveringArg) => {
    console.log('鼠标移出' + JSON.stringify(arg))
}



const events: EventSourceFunc = async (arg, successCallback, _failureCallback) => {
    console.log("原始数据 " + JSON.stringify(arg));
    let req = {
        start: moment(arg.start).format("yyyy-MM-hh"),
        end:moment(arg.end).format("yyyy-MM-hh"),
        timeZone:arg.timeZone
    };
    console.log("请求数据 " + JSON.stringify(req));

    const result:EventInput[] = await invoke("query_calendar_event_source", { value: req });
    console.log("返回结果为",JSON.stringify(result));
    successCallback(result);

    // failureCallback(new Error("请求数据错误"));
}
// const events = async(arg: EventSourceFuncArg, successCallback: (eventInputs: EventInput[]) => void, failureCallback: (error: Error) => void) => {
//     console.log("请求日历数据" + arg);

// }

// 生命周期钩子
onMounted(() => {
    console.log(`init `)
    const calendarEl = document.getElementById('calendar');
    const calendar = new Calendar(calendarEl as HTMLElement, {
        height: "auto",
        plugins: [dayGridPlugin, interactionPlugin],
        initialView: 'dayGridMonth',
        eventClick: handleEventClick,
        dateClick: handleDateClick,
        eventTimeFormat: {
            hour12: false
        },
        //https://fullcalendar.io/docs/events-json-feed#options
        eventSources: [{
            events: events as any,
            id: "1",
            color: 'yellow',
            textColor: 'black'
        }],
        locale: 'zh',
        eventMouseEnter: handleEventMouseEnter,
        eventMouseLeave: handleEventMouseLeave,
    });
    calendar.render();
    watch(
        () => { },
        () => {
            calendar.refetchEvents();
        }
    );
})

</script>


<template>
    <div id="calendar" />
</template>