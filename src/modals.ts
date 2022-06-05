import { bind } from 'svelte-simple-modal'
import { writable } from 'svelte/store'
import Confirm from './component/modal/Confirm.svelte'
import Error from './component/modal/Error.svelte'
import Loading from './component/modal/Loading.svelte'
import NewDeck from './component/modal/NewDeck.svelte'

export const errorModal = writable(null)
export const confirmModal = writable(null)
export const loadingModal = writable(null)
export const newDeckModal = writable(null)

export function showErrorModal(msg?: string, desc?: any) {
    if (desc) console.error(desc)
    errorModal.set(bind(Error, { msg, desc }))
}

export function showConfirmModal(msg: string, handler: () => void) {
    confirmModal.set(bind(Confirm, { msg, handler: (ok: boolean) => { if (ok) handler() } }))
}

export function showConfirmModalPromise(msg: string): Promise<boolean> {
    return new Promise<boolean>(resolve => {
        confirmModal.set(bind(Confirm, { msg, handler: (ok: boolean) => resolve(ok) }))
    })
}

export function showNewDeckModal(): Promise<boolean> {
    return new Promise<boolean>(resolve => {
        newDeckModal.set(bind(NewDeck, { handler: (ok: boolean) => resolve(ok) }))
    })
}

export function showLoadingModal(msg?: string) {
    loadingModal.set(bind(Loading, { msg }))
}

export function hideLoadingModal() {
    loadingModal.set(null)
}