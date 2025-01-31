import { server } from "@/modules"
import { Swap } from "@/modules/solana/swap"

const resource = 'tokens'

export default {
    swap: (swap: Swap): Promise<void> => {
        return new Promise((ok, err) => {
            server.post(resource, 'swap', swap.toSnakeCase())
                .then(r => {
                    console.log(r)
                    ok()
                })
                .catch(e => err(e))
        })
    },

}