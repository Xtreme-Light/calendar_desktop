<script setup lang="ts">
// https://blog.csdn.net/lfwoman/article/details/120177637
// import FullCalendar from '@fullcalendar/vue3'
import { Calendar,  EventClickArg, EventHoveringArg,  EventSourceFunc } from '@fullcalendar/core'
import dayGridPlugin from '@fullcalendar/daygrid'
import interactionPlugin, { DateClickArg } from '@fullcalendar/interaction'
import { onMounted, watch } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

const handleEventClick = (eventClickInfo: EventClickArg) => {
    console.log('any event' + eventClickInfo)
}


const handleDateClick = (arg: DateClickArg) => {
    console.log('date click! ' + arg.dateStr)

}
const handleEventMouseEnter = (arg: EventHoveringArg) => {
    console.log('鼠标移入' + arg)
}

const handleEventMouseLeave = (arg: EventHoveringArg) => {
    console.log('鼠标移出' + arg)
}



const events: EventSourceFunc = async (arg, successCallback, _failureCallback) => {
    console.log("请求日历数据" + arg);
    const req = {
        start:arg.start,
        end:arg.end
    }
    const result = await invoke("query_calendar_event_source",  req );
    console.log("返回结果为" + result);
    successCallback([{ title: 'event1', start: '2023-11-09' }, { title: 'event2', start: '2023-11-10' }]);

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
            color: 'background',
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