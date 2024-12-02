import upsidedown
import json

with open("en-US.json", "r") as f:
    data = json.loads(f.read())

for k, v in data.items():
    data[k] = upsidedown.transform(v).replace("p%", "%d").replace("<u\u0250ds/>", "<span>").replace("<u\u0250ds>", "</span>")

data["name"] = "Ǝuɓlᴉsɥ (Ոdsᴉpǝ ᗡoʍu)"

with open("en-UD.json", "w") as f:
    f.write(json.dumps(data, indent=4))
