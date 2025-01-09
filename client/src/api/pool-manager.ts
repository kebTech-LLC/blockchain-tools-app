import { server } from "@/modules"

const resource = 'pool-manager'

export default {
    getPositions: (): Promise<string> => {
        return new Promise((ok, err) => {
            server.post(resource, 'get-positions', {})
                .then(r => {

                    ok('')
                })
                .catch(e => err(e))
        })
    },

}