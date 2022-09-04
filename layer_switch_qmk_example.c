enum layers
{
    _COLEMAK,
    _QWERTY,
    _NAV,
    _NUM,
    _SYM,
    _FUN,
    _UNI,
    _WIN,
};

layer_state_t layer_state_set_user(layer_state_t state)
{
    int16_t layer = _COLEMAK;

    if (IS_LAYER_ON_STATE(state, _NAV))
    {
        layer = _NAV;
    }
    else if (IS_LAYER_ON_STATE(state, _NUM))
    {
        layer = _NUM;
    }
    else if (IS_LAYER_ON_STATE(state, _SYM))
    {
        layer = _SYM;
    }
    else if (IS_LAYER_ON_STATE(state, _FUN))
    {
        layer = _FUN;
    }
    else if (IS_LAYER_ON_STATE(state, _WIN))
    {
        layer = _WIN;
    }
    else if (IS_LAYER_ON_STATE(state, _UNI))
    {
        layer = _UNI;
    }
    else if (IS_LAYER_ON_STATE(state, _COLEMAK))
    {
        layer = _COLEMAK;
    }
    else if (IS_LAYER_ON_STATE(state, _QWERTY))
    {
        layer = _QWERTY;
    }

    tap_code16(KC_F13 + layer);

    return state;
}
