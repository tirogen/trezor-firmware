import os.path


def add_font(font_name, font, defines, sources):
    if font is not None:
        defines += [
            'TREZOR_FONT_' + font_name + '_ENABLE=' + font,
            'TREZOR_FONT_' + font_name + '_INCLUDE=\\"' + font.lower() + '.h\\"',
        ]
        sourcefile = 'embed/extmod/modtrezorui/fonts/' + font.lower() + '.c'
        if sourcefile not in sources:
            sources.append(sourcefile)


def configure_board(model, env, defines, sources):
    model_r_version = 4

    if model in ('1',):
        board = 'trezor_1.h'
        display = 'vg-2864ksweg01.c'
    elif model in ('T',):
        board = 'trezor_t.h'
        display = 'st7789v.c'
    elif model in ('R',):
        if model_r_version == 3:
            board = 'trezor_r_v3.h'
            display = "ug-2828tswig01.c"
        else:
            board = 'trezor_r_v4.h'
            display = 'vg-2864ksweg01.c'
    else:
        raise Exception("Unknown model")

    defines += [f'TREZOR_BOARD=\\"boards/{board}\\"', ]
    sources += [f'embed/trezorhal/displays/{display}', ]
    env.get('ENV')['TREZOR_BOARD'] = board


def get_model_identifier(model):
    if model == '1':
        return "T1B1"
    elif model == 'T':
        return "T2T1"
    elif model == 'R':
        return "T2B1"
    else:
        raise Exception("Unknown model")


def get_version(file):
    major = 0
    minor = 0
    patch = 0

    if not os.path.exists(file):
        file = os.path.join("..", "..", file)

    with open(file, 'r') as f:
        for line in f:
            if line.startswith('#define VERSION_MAJOR '):
                major = line.split('VERSION_MAJOR')[1].strip()
            if line.startswith('#define VERSION_MINOR '):
                minor = line.split('VERSION_MINOR')[1].strip()
            if line.startswith('#define VERSION_PATCH '):
                patch = line.split('VERSION_PATCH')[1].strip()
        return f'{major}.{minor}.{patch}'
