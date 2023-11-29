import { customRef } from "vue";

/**
 * 节流函数，返回一个延迟更新的引用
 * @param value - 被延迟更新的值
 * @param duration - 更新延迟的时间（默认为1000ms）
 * @returns 一个引用对象，包含get和set方法
 */
export function debounceRef<T>(value: T, duration: number = 1000) {
    return customRef((track, trigger) => {
        let timer: number;
        return {
            /**
             * 获取值并跟踪更新
             */
            get() {
                track()
                return value
            },
            /**
             * 设置新值并触发更新
             * @param newValue - 新值
             */
            set(newValue) {
                clearTimeout(timer)
                timer = setTimeout(() => {
                    value = newValue
                    trigger()
                }, duration)
            }
        }
    })
}


