import { server } from "@/modules"

const resource = 'tokens'

export default {
    swap: (data: any): Promise<any> => {
        return new Promise((ok, err) => {
            server.post(resource, 'swap', data)
                .then(r => ok(r.data))
                .catch(e => err(e))
        })
    },

    register: (): Promise<string> => {
        return new Promise((ok, err) => {
            server.post(resource, 'register', { subscriptions: ['server-info', 'managed-position'] })
                .then(r => {
                    const clientId: string = r.data
                    ok(clientId)
                })
                .catch(e => err(e))
        })
    },

}