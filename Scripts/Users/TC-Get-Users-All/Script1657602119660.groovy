import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper

response6 = WS.sendRequest(findTestObject('Users/GET - Users All'))

def slurper = new JsonSlurper()

def result = slurper.parseText(response6.getResponseBodyContent())

for (int i = 0; i < result.size(); i++) {
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].id', result[i].id)

	WS.verifyElementPropertyValue(response6, ('[' + i) + '].name', result[i].name)

	WS.verifyElementPropertyValue(response6, ('[' + i) + '].username', result[i].username)

	WS.verifyElementPropertyValue(response6, ('[' + i) + '].email', result[i].email)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].address.street', result[i].address.street)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].address.suite', result[i].address.suite)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].address.city', result[i].address.city)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].address.zipcode', result[i].address.zipcode)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].address.geo', result[i].address.geo)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].address.geo.lat', result[i].address.geo.lat)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].address.geo.lng', result[i].address.geo.lng)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].phone', result[i].phone)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].website', result[i].website)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].company.name', result[i].company.name)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].company.catchPhrase', result[i].company.catchPhrase)
	
	WS.verifyElementPropertyValue(response6, ('[' + i) + '].company.bs', result[i].company.bs)
	
}

