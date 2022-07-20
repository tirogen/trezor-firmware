from typing import TYPE_CHECKING

import storage
from trezor import ui
from trezor.enums import ButtonRequestType
from trezor.messages import Success
from trezor.ui.layouts import confirm_action

from .apply_settings import reload_settings_from_storage

if TYPE_CHECKING:
    from trezor import wire
    from trezor.messages import WipeDevice


async def wipe_device(ctx: wire.GenericContext, msg: WipeDevice) -> Success:
    await confirm_action(
        "confirm_wipe",
        title="Wipe device",
        description="Do you really want to\nwipe the device?\n",
        action="All data will be lost.",
        hold=True,
        # TODO hold_danger=True,
        br_code=ButtonRequestType.WipeDevice,
    )

    storage.wipe()
    reload_settings_from_storage()

    return Success(message="Device wiped")
