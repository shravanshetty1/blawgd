// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT MODIFY.
import Shravanshetty1SamacharShravanshetty1SamacharSamachar from './shravanshetty1/samachar/shravanshetty1.samachar.samachar';
export default {
    Shravanshetty1SamacharShravanshetty1SamacharSamachar: load(Shravanshetty1SamacharShravanshetty1SamacharSamachar, 'shravanshetty1.samachar.samachar'),
};
function load(mod, fullns) {
    return function init(store) {
        if (store.hasModule([fullns])) {
            throw new Error('Duplicate module name detected: ' + fullns);
        }
        else {
            store.registerModule([fullns], mod);
            store.subscribe((mutation) => {
                if (mutation.type == 'common/env/INITIALIZE_WS_COMPLETE') {
                    store.dispatch(fullns + '/init', null, {
                        root: true
                    });
                }
            });
        }
    };
}
