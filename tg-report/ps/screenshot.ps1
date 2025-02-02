Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

# маштаб интерфейса 125%
[float]$scale = 1.25
$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$scaledBounds = New-Object System.Drawing.Rectangle(0, 0, [int]($bounds.Width * $scale), [int]($bounds.Height * $scale))
$bitmap = New-Object System.Drawing.Bitmap $scaledBounds.Width, $scaledBounds.Height
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)

$graphics.CopyFromScreen($bounds.Location, [System.Drawing.Point]::Empty, $scaledBounds.Size)
$bitmap.Save("$env:USERPROFILE\Desktop\screen.png", [System.Drawing.Imaging.ImageFormat]::Png)

$graphics.Dispose()
$bitmap.Dispose()