-- lua entrypoint for the controller

local x, y, z = badge.sensor.accel()

function on_enter(root)
  local lbl = badge.ui.label(root, "controller")
  lbl:set_font_size("large")
  lbl:align("center", 0, 0)
end

function on_button(button, kind)
  local contact = badge.input.BUTTON
  local contact_kind = badge.input.KIND

  -- user pressed UP
  if button == contact.UP then
    if kind == contact_kind.PRESSED then
      badge.sys.log("1:0")
    else
      badge.sys.log("1:1")
    end

  -- user pressed LEFT
  elseif button == contact.LEFT then
    if kind == contact_kind.PRESSED then
      badge.sys.log("2:0")
    else
      badge.sys.log("2:1")
    end

  -- user presed DOWN
  elseif button == contact.DOWN then
    if kind == contact_kind.PRESSED then
      badge.sys.log("3:0")
    else
      badge.sys.log("3:1")
    end

  -- user pressed RIGHT
  elseif button == contact.RIGHT then
    if kind == contact_kind.PRESSED then
      badge.sys.log("4:0")
    else
      badge.sys.log("4:1")
    end

  -- user pressed HOME
  elseif button == contact.HOME then
    if kind == contact_kind.PRESSED then
      badge.sys.log("5")
    else
    end
  -- user pressed START
  elseif button == contact.START then
    if kind == contact_kind.PRESSED then
      badge.sys.log("6:0")
    else
      badge.sys.log("6:1")
    end

  -- user pressed B
  elseif button == contact.B then
    if kind == contact_kind.PRESSED then
      badge.sys.log("7:0")
    else
      badge.sys.log("7:1")
    end

  -- user pressed A
  elseif button == contact.A then
    if kind == contact_kind.PRESSED then
      badge.sys.log("8:0")
    else
      badge.sys.log("8:1")
    end

  else
    return

  end
  
end