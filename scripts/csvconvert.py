import csv

#ask for filename
filename = input("What is the path of your file? ")

#ask how many non-integer fields
num_nonint_fields = int(input("\nHow many non-integer columns? "))

decimal = pow(10,int(input("\nBiggest decimal? ")))
print("Factor of " + str(decimal))

#ask their positions [0...x]
indexes = []
print("\nInput their indexes (0...x) and press ENTER after each one:")
for x in range(0,num_nonint_fields):
	i = input("")
	indexes.append(int(i))


#do operations and write to new file

#get all rows
rows = []
fields = []
with open(filename, 'r') as csvread:
	csvreader = csv.reader(csvread)
	fields = next(csvreader)
	for row in csvreader:
		rows.append(row)
	print("\nRead initial CSV file. Performing row operations...")

#do operation on each row and write to new csv
for row in rows:
	for x in range(0,num_nonint_fields):
		row[indexes[x]] = int(float(row[indexes[x]]) * decimal)
		#print(row[indexes[x]])
print("Row operations complete. Writing to new CSV file...")

#write to new file
newfilename = filename+"onlyint.csv"
with open(newfilename, 'w') as csvwrite:
	csvwriter = csv.writer(csvwrite)
	csvwriter.writerow(fields)
	csvwriter.writerows(rows)

#print success message
print("Successfully created file and performed row operations!")