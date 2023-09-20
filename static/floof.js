function FloofReactivity() {
    const properties = {};
    const effects = [];

    function createProperty(name, initialValue) {
        properties[name] = initialValue;
    }

    function getValue(name) {
        return properties[name];
    }

    function setValue(name, value) {
        if (properties[name] !== value) {
            properties[name] = value;
            triggerEffects(name);
        }
    }

    function createEffect(effectFn) {
        effects.push(effectFn);
    }

    function triggerEffects(changedProperty) {
        effects.forEach((effectFn) => {
            effectFn(changedProperty);
        });
    }

    return {
        createProperty,
        getValue,
        setValue,
        createEffect,
    };
}