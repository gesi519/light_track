import os
from PIL import Image

# 创建输出目录
output_dir = "converted_pngs"
os.makedirs(output_dir, exist_ok=True)

# 遍历当前目录下的所有文件
for filename in os.listdir("."):
    if filename.lower().endswith(".ppm"):
        try:
            # 打开 ppm 图像
            with Image.open(filename) as img:
                # 构建输出路径（例如 image1.ppm → converted_pngs/image1.png）
                output_path = os.path.join(output_dir, os.path.splitext(filename)[0] + ".png")
                img.save(output_path)
                print(f"✅ 成功转换：{filename} → {output_path}")
        except Exception as e:
            print(f"转换失败：{filename}，错误：{e}")
