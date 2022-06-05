import type { Field } from 'svelte-forms/types'
import { get, Readable, Writable } from 'svelte/store'

export type WritableField = Writable<Field<any>>
export type ReadableField = Readable<Field<any>>

export function prettifyFormErrors(form: any): string {
    const errors = get<any>(form).errors
    return errors.map((it: string) =>
        (it.substring(0, 1).toUpperCase() + it.substring(1))
            .split('.').join(' ')
    ).join('. ')
}

export function fieldValue<T>(form: any, name: string): T {
    const val = get<any>(form.getField(name)).value
    return (typeof(val) === 'string' ? val.trim() : val) as T
}

export function writableField(form: any, name: string): WritableField {
    return form.getField(name) as WritableField
}

export function readableField(form: any, name: string): ReadableField {
    return form.getField(name) as ReadableField
}

export function setField(form: any, name: string, value: any) {
    const field = form.getField(name)
    get<any>(field).value = value
    field.update((f: any) => f)
}