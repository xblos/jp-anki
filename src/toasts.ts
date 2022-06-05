import { toast } from '@zerodevx/svelte-toast'

export function showSuccessToast(msg: string) {
    toast.push(msg, {
        duration: 2000,
        theme: {
            '--toastBackground': '#48BB78',
            '--toastBarBackground': '#2F855A',
        }
    })
}